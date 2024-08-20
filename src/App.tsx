import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [types, setTypes] = useState<string[]>([]);
  const [contents, setContents] = useState<
    {
      format: string;
      data: string;
    }[]
  >([]);

  async function read() {
    setContents(await invoke("read"));
  }

  async function watch() {
    await invoke("watch");
  }

  async function stopWatch() {
    await invoke("stop_watch");
  }

  useEffect(() => {
    const unlisten = listen<string[]>("clipboard_change", async (event) => {
      setTypes(event.payload);
      await read();
    });
    return () => {
      unlisten.then((u) => u());
    };
  }, []);

  return (
    <div className="container">
      <p>剪贴板类型：</p>
      {types.map((type, index) => (
        <p key={index}>{type}</p>
      ))}
      <p>剪贴板内容：</p>
      {contents.map((content, index) => (
        <div key={index} style={{ background: "rgba(0,0,0,0.1)", minHeight: "100px" }}>
          <p>格式：{content.format}</p>
          <p>内容：{content.data}</p>
        </div>
      ))}
      <div style={{ display: "flex", gap: "24px", justifyContent: "center", marginTop: "50px" }}>
        <button onClick={read}>Read</button>
        <button onClick={watch}>startWatch</button>
        <button onClick={stopWatch}>stopWatch</button>
      </div>
    </div>
  );
}

export default App;
