import { Smile, Angry, Radiation, Grab, BicepsFlexed } from "lucide-react";
import "./App.css";
import Playback from "./Playback";

const robotIp = "10.33.85.8";

function executeCommand(command: string) {
  fetch(`http://localhost:3000/${command}`, {
    method: "POST",
  }).catch((err) => console.log(`Failed to execute ${command}`, err));
}

function App() {
  const handleClick = () => {};

  return (
    <div className="flex justify-between w-screen h-screen px-8">
      <div className="w-48 p-8 h-screen flex flex-col justify-between items-center">
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => executeCommand("muscles")}
        >
          <BicepsFlexed />
        </button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => executeCommand("dab")}
        >
          <Grab />
        </button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          <Angry />
        </button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          <Smile />
        </button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          <BicepsFlexed />
        </button>
        <button
          className="h-20 w-20 z-30 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          <Radiation />
        </button>
      </div>

      {/* right side */}

      <div className="w-48 p-8 h-screen flex flex-col justify-between items-center">
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Smile
        </button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        ></button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          <Smile />
        </button>

        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
        <button
          className="h-20 w-20 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
        <button
          className="h-20 w-20 z-30 rounded-lg flex justify-center items-center bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
      </div>

      <div className="flex flex-col justify-between h-full p-8 gap-8">
        <Playback
          robotIpAddr={robotIp}
          className="flex flex-grow justify-center items-center rounded-lg"
        />
        <div className="w-full flex justify-center items-end space-x-6">
          <button
            className="h-20 w-20 rounded-lg  bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
          <button
            className="h-20 w-20 rounded-lg bg-violet-500 transition-all hover:scale-110 hover:bg-violet-900"
            onClick={() => handleClick()}
          >
            <Smile />
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
