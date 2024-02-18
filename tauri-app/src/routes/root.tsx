import { Link } from "react-router-dom";

function Root() {

  return (
    <div className="">
      <Link to="/debug" className="daisy-btn daisy-btn-link">
        Debug
      </Link>
      <Link to="/config" className="daisy-btn daisy-btn-link">
        Config
      </Link>
      <Link to="/run" className="daisy-btn daisy-btn-link">
        Run
      </Link>
    </div>
  );
}

export default Root;
