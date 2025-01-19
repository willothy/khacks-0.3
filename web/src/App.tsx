import { useState } from "react";
import { Smile, Angry, Radiation, Grab, BicepsFlexed } from 'lucide-react';
import "./App.css";


function App() {
  const handleClick = () => {};

  return (
    <div className= "flex justify-between w-screen px-8"> 
      <div className="w-48 p-8 h-screen flex flex-col justify-around items-center">
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
          <Smile />
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
          <Grab />
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

      <div className="w-48 p-8 h-screen flex flex-col justify-around items-center">
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

      <div className="w-full h-screen p-8 flex justify-center items-end space-x-6">
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
  );
}

export default App;

