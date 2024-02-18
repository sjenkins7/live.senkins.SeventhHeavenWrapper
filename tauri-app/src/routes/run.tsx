import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'


function StatusStep(props: StatusUpdate) {
  console.log(props);
  if (!props.running) {
    if (props.success) {
      return <pre data-prefix="✓" className="text-success"><code>{props.step}</code></pre>
    } else {
      return <pre data-prefix="✘" className="text-error"><code>{props.step}</code></pre>
    }
  } else {
    return <pre data-prefix=">"><code>{props.step}</code></pre>
  }
}

interface StatusUpdate {
  step: String,
  running: boolean,
  success: boolean
}

function mergeStatus(toMerge: StatusUpdate, status: StatusUpdate[], callback: Dispatch<SetStateAction<StatusUpdate[]>>) {
  
  var result = Object.assign([], status) as StatusUpdate[];
  var foundIndex = result.findIndex(x => x.step == toMerge.step);
  if (foundIndex == -1) {
    result.push(toMerge)
  } else {
    result[foundIndex] = toMerge;
  }
  callback(result)
}


function Run() {
  const [installStatus, setInstallStatus] = useState<StatusUpdate[]>([]);

  useEffect(() => {
    // Delay to avoid double triggers in dev mode, due to React.StrictMode
    let timer: ReturnType<typeof setTimeout>;
    timer = setTimeout(async () => {
      // TODO - move this to a button that triggers install & moves to this page
      invoke("install_run");
    }, 500);

    return () => {
      clearTimeout(timer);
      // invoke("install_cancel")
    }
  }, [])

  useEffect(() => {
    const unlisten = listen<StatusUpdate>('install_progress', (event) => {
      console.log('Received event:', event.payload);
      mergeStatus(event.payload, installStatus, setInstallStatus);
    });

    return () => {
      unlisten.then(f => f());
    };
  }, [installStatus, setInstallStatus]);


  let statuses = installStatus.map(e => {
    return <StatusStep {...e} />
  })

  let errored = installStatus.find(i => i.success === false && i.running === false);

  let errorMessage;
  if (errored) {
    errorMessage = (
      <div className="daisy-alert daisy-alert-error">
        <svg xmlns="http://www.w3.org/2000/svg" className="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <span>Error! Task failed successfully.</span>
      </div>
    )
  }

  return (
    <div className="grid h-screen place-items-center w-1/2 m-auto">
      <div className="daisy-mockup-code w-full">
        {...statuses}
      </div>
      {errorMessage}
    </div>
  );
}


export default Run;
