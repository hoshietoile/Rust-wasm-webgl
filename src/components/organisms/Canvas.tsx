import { useAtom } from 'jotai';
import React, { ComponentPropsWithoutRef, forwardRef, useCallback, useEffect, useImperativeHandle, useRef, useState } from 'react'

import { gameStateAtom } from '../../stores/gameStore';
import { threadsStateAtom } from '../../stores/threadStore';
import init, { init_screen, Screen } from '../../../wasm/pkg/wasm'

export interface CanvasHandler {
  updateCanvasState: () => void;
  gameStart: () => void;
  gameTmpStop: () => void;
  gameStop: () => void;
  addThread: () => void;
}

type CanvasProps = ComponentPropsWithoutRef<any> & {

}

export const Canvas: React.FC<CanvasProps> = forwardRef<CanvasHandler>(({}, ref) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const glInstanceRef = useRef<Screen | null>(null);
  const animateRef = useRef<boolean>(false);
  const requestRef = useRef<number>(0);

  const [canvasState] = useAtom(gameStateAtom);

  const currentThreadId = useRef<number | null>(null);
  // const [threadIds, setThreadIds] = useState<number[]>([]);

  const [threadsState, setThreadsState] = useAtom(threadsStateAtom);

  /**
   * 各フレームの実行
   * @param time 
   * @returns 
   */
  const doFrame = (time: number) => {
    if (!glInstanceRef.current) return;
    glInstanceRef.current.do_frame(time);
    requestRef.current = requestAnimationFrame(doFrame);
  }

  /** canvas初期化処理. */
  const initGl = () => {
    glInstanceRef.current = init_screen({
      ...canvasState
    });
    console.log(glInstanceRef.current)
    const _ids = glInstanceRef.current.get_thread_ids();
    const ids = Array.from(_ids); // Uint32Array -> number[]変換
    currentThreadId.current = ids[0];
    // setThreadIds(ids);
    setThreadsState(ids)
    // glInstanceRef.current.do_frame(0);
    // doFrame(0);
  }

  /** 初期化時処理 */
  useEffect(() => {
    const initialize = async () => {
      await init();
      initGl(); 
    }
    initialize();
  }, [])

  /** スレッドの追加/更新 追加時はcurrentThreadId = null */
  const upsertThread = () => {
    if (glInstanceRef.current) {
      glInstanceRef.current.upsert_thread_setting(currentThreadId.current || undefined, {...canvasState});
      const _ids = glInstanceRef.current.get_thread_ids();
      const ids = Array.from(_ids); // Uint32Array -> number[]変換
      setThreadsState(ids)
      // setThreadIds(ids);
    }
  }

  /**
   * スレッドの追加(TODO: ほかのコンポーネントに役割を移動)
   */
  const addThread = () => {
    if (glInstanceRef.current) {
      glInstanceRef.current.upsert_thread_setting(undefined, { ...canvasState });
      const _ids = glInstanceRef.current.get_thread_ids();
      const ids = Array.from(_ids);
      setThreadsState(ids)
    }
  }

  /** 親コンポーネントから命令的に事項するインタフェース */
  useImperativeHandle(ref, () => ({
    updateCanvasState() {
      upsertThread(); 
    },
    gameStart() {
      if (animateRef.current === true) return;
      animateRef.current = true;
      doFrame(0);
    },
    gameTmpStop() {
      animateRef.current = false;
      cancelAnimationFrame(requestRef.current);
    },
    gameStop() {
      animateRef.current = false;
      initGl();
    },
    addThread() {
      addThread();
    }
  })); 

  return (
    <>
    {JSON.stringify(threadsState)}
    <canvas
      id={canvasState.canvas_id}
      ref={canvasRef}
      width={canvasState.width}
      height={canvasState.height}
    />
    </>
  );
});