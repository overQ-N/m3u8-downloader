import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { PlusIcon } from "@heroicons/react/24/solid";

const Aside = () => {
  /** 新建任务弹窗 */
  const [taskVis, setTaskVis] = useState(false);
  /**  */
  const [value, setValue] = useState("");
  const invokeParse = () => {
    invoke("parser_curl", { value });
    // invoke("greet", { curl: 11 });
    console.log("a23");
  };
  return (
    <div className="flex flex-col items-center w-20 bg-theme-default">
      <PlusIcon
        className="w-8 h-8 text-white cursor-pointer hover:bg-text-white"
        onClick={() => setTaskVis(true)}
      />

      {/* {
        taskVis &&
    } */}

      <div className="fixed inset-0 bg-gray-alpha-75">
        <div className="absolute w-4/5 p-4 -translate-x-1/2 -translate-y-1/2 bg-white rounded-lg shadow-md top-1/2 left-1/2 h-1/2">
          <textarea
            className="w-full h-24 p-4 border rounded-lg border-gray-4 focus:outline-theme-default"
            value={value}
            onChange={(ev) => setValue(ev.target.value)}
          ></textarea>
          <div className="absolute bottom-0 left-0 right-0 flex items-center justify-end px-4 h-15 bg-theme-alpha-10">
            <button>取消</button>
            <button
              className="text-white bg-theme-default"
              onClick={invokeParse}
            >
              提交
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Aside;
