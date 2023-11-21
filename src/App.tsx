import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Aside from "./components/aside";
import SubNav from "./components/subnav";

function App() {
  const [curl, setCurl] = useState("");
  const [value, setValue] = useState("");
  async function parseCurl() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // setGreetMsg();
    // debugger;
    setValue(await invoke("greet", { name: "name", age: "age", curl }));
    // console.log(await invoke("greet", { name: "name", age: "age", curl }));
  }

  return (
    <div className="flex w-full h-full">
      <Aside />
      <SubNav />
      <div className="flex-1">subnav</div>
    </div>
  );
}

export default App;
