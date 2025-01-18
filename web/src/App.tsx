import { useState } from "react";

import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      Counter {count}{" "}
      <button
        className="m-2 p-2 bg-blue-600 text-white rounded hover:bg-blue-500"
        onClick={() => setCount((count) => count + 1)}
      >
        Increment
      </button>
    </>
  );
}

export default App;
