import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function Root() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [enabled, setEnabled] = useState(false)

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="">
      <h1 className="text-3xl font-bold underline">
        Hello world!
      </h1>
      <input type="checkbox" className="daisy-toggle daisy-toggle-success" checked={enabled} onChange={(e) => setEnabled(e.currentTarget.checked)}/>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          className="daisy-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit" className="daisy-btn">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default Root;
