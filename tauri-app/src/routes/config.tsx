function Config() {
  return (
    <div className="">
      <div className="daisy-form-control w-full max-w-xs">
        <label className="daisy-label">
          <span className="daisy-label-text">Install Location</span>
        </label>
        <select className="daisy-select daisy-select-bordered" disabled>
          <option>~/.var.app/</option>
        </select>
      </div>
    </div>
  );
}

export default Config;
