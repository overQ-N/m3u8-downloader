import { useEffect, useState } from "react";
import { Toaster } from "react-hot-toast";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Aside from "./components/aside";
import SubNav from "./components/subnav";

// interface IProps {
//   theme: {
//     r: number;
//     g: number;
//     b: number;
//     a: number;
//   };
// }
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

  const createStyleSheet = () => {
    const head = document.head || document.getElementsByTagName("head")[0];
    const style = document.createElement("style");
    head.appendChild(style);
    return style.sheet;
  };

  const theme = { r: 0, g: 0, b: 0, a: 1 };
  /** 注入主题色 */
  useEffect(() => {
    createStyleSheet()?.insertRule(`
    :root {
      --theme-color: rgba(${theme.r}, ${theme.g}, ${theme.b});
${[6, 10, 20, 30, 40, 50, 60, 70, 75, 80, 90]
  .map(
    (item) =>
      `--theme-color-alpha-${item}: rgba(${theme.r}, ${theme.g}, ${theme.b}, ${
        item / 100
      })`
  )
  .join(";")}
  `);
  }, []);

  return (
    <div className="flex w-full h-full">
      <Aside />
      <SubNav />
      <div className="flex-1">subnav</div>
      <Toaster containerClassName="z-[99999]" />
    </div>
  );
}

export default App;
