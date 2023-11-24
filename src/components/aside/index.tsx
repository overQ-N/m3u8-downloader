import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import toast from "react-hot-toast";
import { PlusIcon, FolderIcon } from "@heroicons/react/24/solid";
import { IResponse } from "../../typings";
import { message, open, confirm } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { useClickOutside } from "../../hooks";
import { exists } from "@tauri-apps/api/fs";

/** 默认输出的文件 */
const DEFAULT_OUTPUT_FILE_NAME = "output.mkv";
const Aside = () => {
  const [taskVis, setTaskVis] = useState(false);
  /**  */
  const [value, setValue] = useState("");
  /** Dialog Element */
  const _dialog = useRef<HTMLDialogElement>(null);
  /** Modal */
  const _modal = useRef<HTMLDivElement>(null);
  /** 用户系统默认下载地址 */
  const [downloadPath, setDownloadPath] = useState("");
  /** 输出文件名 */
  const [outputFile, setOutputFile] = useState("");

  /** 获取用户系统下载文件夹path */
  const getUserDownloadsPath = async () => {
    setDownloadPath(await downloadDir());
  };
  useEffect(() => {
    getUserDownloadsPath();
  }, []);

  useEffect(() => {
    const fn = (ev: MouseEvent) => {
      if (!_modal.current?.contains(ev.target as HTMLElement)) {
        setTaskVis(false);
      }
    };
    document.addEventListener("mousedown", fn);
    return () => document.removeEventListener("mousedown", fn);
  }, []);

  const invokeParse = async (output: string) => {
    const resp = await invoke<IResponse>("parser_curl", {
      value,
      outputFile: output,
      downloadPath,
    });
    if (resp.code === 200) {
      _dialog.current?.close();
    } else {
      toast.error(resp.msg, {});
      // message(resp.msg, { type: "error" });
    }
  };
  const submit = async () => {
    /** Detect file is exists */
    let output = outputFile || DEFAULT_OUTPUT_FILE_NAME;
    const resp = await invoke<IResponse>("detect_file", {
      outputFile: output,
      downloadPath,
    });
    if (resp.code === 200) {
      _dialog.current?.close();
    } else {
      if (resp.code === 409) {
        const confirmed = await confirm(resp.msg, {
          type: "warning",
        });
        if (!confirmed) return;
        await invoke("parser_curl", {
          value,
          outputFile: output,
          downloadPath,
        });
      } else {
        toast.error(resp.msg);
      }
    }
  };

  /** 选择文件下载路径 */
  const selectDownloadPath = async () => {
    try {
      const selected = await open({
        directory: true,
      });
      console.log("selected", selected);
      if (selected) {
        setDownloadPath(selected as string);
      }
    } catch (error) {
      console.log("error", error);
    }
  };
  return (
    <div className="flex flex-col items-center w-20 bg-theme-default">
      <label htmlFor="aside-modal">
        <PlusIcon
          className="w-8 h-8 text-white cursor-pointer hover:bg-text-white"
          onClick={() => setTaskVis(true)}
        />
      </label>

      {taskVis && (
        <div
          className="opacity-100 modal"
          style={{
            pointerEvents: "unset",
          }}
        >
          <div className="modal-box" ref={_modal}>
            <textarea
              placeholder="Curl链接"
              className="w-full p-0 textarea textarea-bordered"
              value={value}
              onChange={(ev) => setValue(ev.target.value)}
            ></textarea>
            <div className="flex items-center mt-4">
              <div className="w-1/5">文件名: </div>
              <div className="flex items-center flex-1 ">
                <input
                  type="text"
                  className="w-full input input-bordered"
                  value={outputFile}
                  placeholder={DEFAULT_OUTPUT_FILE_NAME}
                  onChange={(ev) => setOutputFile(ev.target.value)}
                />
              </div>
            </div>
            <div className="flex items-center mt-4">
              <div className="w-1/5">下载路径: </div>
              <div className="flex items-center flex-1 px-4 inner-border after:border-gray-4 after:rounded-lg focus-within:outline-2 focus-within:outline-offset-2 focus-within:outline-base-content/20 focus-within:outline focus-within:rounded-lg">
                <input
                  type="text"
                  className="w-full h-12 focus:outline-none"
                  value={downloadPath}
                  onChange={(ev) => setDownloadPath(ev.target.value)}
                />
                <FolderIcon
                  className="w-5 h-5 cursor-pointer text-gray-6"
                  onClick={selectDownloadPath}
                />
              </div>
            </div>
            <div className="modal-action">
              <button className="btn btn-primary" onClick={submit}>
                提交
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Aside;
