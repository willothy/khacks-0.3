import { useState } from "react";
import { Smile } from 'lucide-react';
import "./App.css";


function App() {
  const handleClick = () => {};

  return (
    <>
      <div className="absolute top-0 right-0 w-48 p-8 h-screen flex flex-col justify-around">
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          <   Smile />
        </button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        ></button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
      </div>

{/* right side */}

      <div className="absolute bottom-0 left-0 w-48 p-8 h-screen flex flex-col justify-around">
       <button 
        className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
        onClick={() => handleClick()}
      >
        Smile
      </button>
      <button
        className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
        onClick={() => handleClick()}
      ></button>
      <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
      <button
        className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
        onClick={() => handleClick()}
      >
        Hi
      </button>
      <button
        className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
        onClick={() => handleClick()}
      >
        Hi
      </button>

      
     
      </div> 
    </>
  );
}

export default App;

