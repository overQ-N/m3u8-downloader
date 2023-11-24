import { RefObject, useEffect } from "react";

/** 点击元素外部触发 */
const useClickOutside = (ref: RefObject<HTMLElement>, callback: Function) => {
  useEffect(() => {
    const fn = (ev: MouseEvent) => {
      if (!ref.current?.contains(ev.target as HTMLElement)) {
        callback();
      }
    };
    document.addEventListener("mousedown", fn);
    return () => document.removeEventListener("mousedown", fn);
  }, []);
};
export default useClickOutside;
