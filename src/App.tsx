import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [clipTxt, setClipTxt] = useState("");

  async function read() {
    setClipTxt(await invoke("read"));
  }

  async function watch() {
    await invoke("watch");
  }

  async function stopWatch() {
    await invoke("stop_watch");
  }

  useEffect(() => {
    const unlisten = listen<string>("clipboard_change", (event) => {
      setClipTxt(event.payload);
    });
    return () => {
      unlisten.then((u) => u());
    };
  }, []);

  return (
    <div className="container">
      <p>剪贴板内容：</p>
      <p style={{ background: "rgba(0,0,0,0.1)", minHeight: "100px" }}>{clipTxt}</p>
      <div style={{ display: "flex", gap: "24px", justifyContent: "center", marginTop: "50px" }}>
        <button onClick={read}>Read</button>
        <button onClick={watch}>startWatch</button>
        <button onClick={stopWatch}>stopWatch</button>
      </div>
    </div>
  );
}

export default App;
