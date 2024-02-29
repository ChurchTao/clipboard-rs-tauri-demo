import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from '@tauri-apps/api/event';

function App() {

  const [clipTxt,setClipTxt] = useState("");

  async function read() {
    setClipTxt(await invoke("read"))
  }

  async function watch() {
    await invoke('watch');
  }

  useEffect(()=>{
    watch();
    const unlisten = listen('clipboard-change', (event) => {
      read()
    });
  },[])

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <button onClick={read}>Read</button>


      <p>{clipTxt}</p>
    </div>
  );
}

export default App;
