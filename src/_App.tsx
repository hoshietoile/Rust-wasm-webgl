import { useEffect, useState, useRef, useCallback, useReducer } from 'react'
import './App.css'
import init, { init_gl, Screen } from '../wasm/pkg/wasm'

interface CanvasState {
  id: string;
  width: number;
  height: number;
  diskNum: number;
  diskSize: number;
  type: number;
  shotSpeed: number;
  shotWayNum: number;
  shotInterval: number;
  shotBehavior: number;
  speedChangePer: number,
  speedChangeInterval: number,
}

const canvasState: CanvasState = {
  id: "canvas",
  width: 400,
  height: 400,
  diskNum: 100,
  diskSize: 16,
  type: 0,
  shotSpeed: 1,
  shotWayNum: 6,
  shotInterval: 1000,
  shotBehavior: 0,
  speedChangePer: 0.1,
  speedChangeInterval: 100,
};

const ACTION_TYPES = [
  "WINDOW_CHANGE",
  "DISK_SIZE_CHANGE",
  "DISK_NUM_CHANGE",
  "COLLISION_CHANGE",
  "TYPE_CHANGE",
  "WAY_NUM_CHANGE",
  "SHOT_INTERVAL_CHANGE",
  "SHOT_BEHAVIOR_CHANGE",
  "SHOT_SPEED_CHANGE",
  "SPEED_CHANGE_PER_CHANGE",
  "SPEED_CHANGE_INTERVAL_CHANGE",
] as const;

type ADT<Ident extends string, T> = {
  [K in keyof T]: Record<Ident, K> & T[K];
}[keyof T];

type DispatchActions<Ks extends keyof any, T extends Record<Ks, any>> = ADT<"type", T>;

type ActionTypes = typeof ACTION_TYPES[number];

type Actions = DispatchActions<
  ActionTypes,
  {
    WINDOW_CHANGE: { payload: string };
    DISK_SIZE_CHANGE: { payload: string };
    DISK_NUM_CHANGE: { payload: string };
    COLLISION_CHANGE: { payload: boolean };
    TYPE_CHANGE: { payload: string };
    WAY_NUM_CHANGE: { payload: string };
    SHOT_INTERVAL_CHANGE: {payload: string};
    SHOT_BEHAVIOR_CHANGE: {payload: string};
    SHOT_SPEED_CHANGE: {payload: string};
    SPEED_CHANGE_PER_CHANGE: {payload: string};
    SPEED_CHANGE_INTERVAL_CHANGE: {payload: string};
  }
>;

const reducer = (state: CanvasState, action: Actions) => {
  switch (action.type) {
    case ACTION_TYPES[0]:
      return { ...state, width: parseInt(action.payload, 10), height: parseInt(action.payload, 10) };
    case ACTION_TYPES[1]:
      return { ...state, diskSize: parseInt(action.payload, 10) };
    case ACTION_TYPES[2]:
      return { ...state, diskNum: parseInt(action.payload, 10) };
    case ACTION_TYPES[3]:
      return { ...state, collision: action.payload };
    case ACTION_TYPES[4]:
      return { ...state, type: parseInt(action.payload, 10) };
    case ACTION_TYPES[5]:
      const v = parseInt(action.payload, 10);
      if (isNaN(v)) return { ...state };
      return { ...state, shotWayNum: v};
    case ACTION_TYPES[6]:
      return { ...state, shotInterval: parseInt(action.payload, 10) };
    case ACTION_TYPES[7]:
      return { ...state, shotBehavior: parseInt(action.payload, 10)};
    case ACTION_TYPES[8]:
      return { ...state, shotSpeed: parseInt(action.payload, 10)};
    case ACTION_TYPES[9]:
      return { ...state, speedChangePer: parseFloat(action.payload)};
    case ACTION_TYPES[10]:
      return { ...state, speedChangeInterval: parseInt(action.payload, 10)};
    default:
      return state;
  }
}

function App() {
  const t = useRef<number>(0);
  const glInstance = useRef<Screen | null>(null);
  const req = useRef<number | null>(null);
  const [animate, setAnimate] = useState<boolean>(false);
  const [state, dispatch] = useReducer(reducer, canvasState)

  const doFrame = (time: number) => {
    if (!glInstance.current) return;
    glInstance.current.do_frame(time);
    req.current = requestAnimationFrame(doFrame);
  }

  const initGl = () => {
    glInstance.current = init_gl({
      canvas_id: state.id,
      disk_num: state.diskNum,
      width: state.width,
      height: state.height,
      disk_size: state.diskSize,
      shot_type: state.type,
      shot_speed: state.shotSpeed,
      shot_way_num: state.shotWayNum,
      shot_interval: state.shotInterval,
      shot_behavior: state.shotBehavior,
      speed_change_per: state.speedChangePer,
      speed_chane_interval: state.speedChangeInterval,
    });
    glInstance.current.do_frame(0);
  }

  useEffect(() => {
    const initialize = async () => {
      await init();
      initGl(); 
    }
    initialize();
  }, [])

  useEffect(() => {
    if (!glInstance.current) return;
    initGl();
  }, [glInstance, state])

  const toggleAnimationState = useCallback(() => {
    if (animate === true && req.current !== null) {
      setAnimate(false);
      cancelAnimationFrame(req.current);
      req.current = null;
    } else {
      setAnimate(true);
      doFrame(0);
    }
  }, [animate]);

  
  return (
    <div className="App">
      <div>
        <button onClick={toggleAnimationState}>
          {animate === true ? 'stop' : 'start'}
        </button>
        <select value={state.diskSize} onChange={(e) => dispatch({ type: "DISK_SIZE_CHANGE", payload: e.target.value })}>
          <option value="4">4</option>
          <option value="8">8</option>
          <option value="16">16</option>
          <option value="32">32</option>
          <option value="64">64</option>
        </select>
        <select value={state.diskNum} onChange={(e) => dispatch({ type: "DISK_NUM_CHANGE", payload: e.target.value })}>
          <option value="10">10</option>
          <option value="100">100</option>
          <option value="1000">1000</option>
          <option value="10000">10000</option>
          <option value="50000">50000</option>
          <option value="100000">100000</option>
        </select>
        <select value={state.height} onChange={(e) => dispatch({ type: "WINDOW_CHANGE", payload: e.target.value })}>
          <option value="200">200</option>
          <option value="400">400</option>
          <option value="600">600</option>
          <option value="800">800</option>
          <option value="1000">1000</option>
        </select>

        {/* TODO: 射出位置 */}
        {/* TODO: 重力 */}
        {/* TODO: 回転速度 */}
        {/* TODO: 分裂団 */}

        <select value={state.type} onChange={(e) => dispatch({ type: "TYPE_CHANGE", payload: e.target.value })}>
          <option value="0">ランダム</option>
          <option value="1">放射状</option>
          <option value="2">回転</option>
          <option value="3">渦巻</option>

          {/* <option value="2">四方</option> */}
          {/* <option value="6  0"></option>
          <option value="8  0"></option>
          <option value="1000"></option> */}
        </select>
        <input type="number" value={state.shotWayNum} onChange={(e) => dispatch({ type: "WAY_NUM_CHANGE", payload: e.target.value })} />
        <select value={state.shotInterval} onChange={(e) => dispatch({ type: "SHOT_INTERVAL_CHANGE", payload: e.target.value })}>
          {Array(10).fill(null).map((_, idx) => (idx + 1) * 100).map((v, index) => (
            <option key={index} value={v}>{v}</option>
          ))}
        </select>

        <label>弾挙動</label>
        <select value={state.shotBehavior} onChange={(e) => dispatch({ type: "SHOT_BEHAVIOR_CHANGE", payload: e.target.value })}>
          <option value="0">ノーマル</option>
          <option value="1">加速</option>
          <option value="2">減速</option>
          <option value="3">反射</option>
          <option value="4">角度変化</option>
        </select>

        <label>弾速</label>
        <select value={state.shotSpeed} onChange={(e) => dispatch({ type: "SHOT_SPEED_CHANGE", payload: e.target.value })}>
          <option value="1">1</option>
          <option value="2">2</option>
          <option value="3">3</option>
          <option value="4">4</option>
        </select>
            
        <label>速度の変化量</label>
        <select value={state.speedChangePer} onChange={(e) => dispatch({ type: "SPEED_CHANGE_PER_CHANGE", payload: e.target.value })}>
          <option value="0.1">0.1</option>
          <option value="0.25">0.25</option>
          <option value="0.5">0.5</option>
          <option value="0.75">0.75</option>
          <option value="1">1</option>
          <option value="2">2</option>
          <option value="3">3</option>
          <option value="4">4</option>
          <option value="5">5</option>
        </select>

        <label>速度の変化間隔</label>
        <select value={state.speedChangeInterval} onChange={(e) => dispatch({ type: "SPEED_CHANGE_INTERVAL_CHANGE", payload: e.target.value })}>
          <option value="10">10</option>
          <option value="20">20</option>
          <option value="30">30</option>
          <option value="40">40</option>
          <option value="50">50</option>
          <option value="60">60</option>
          <option value="70">70</option>
          <option value="80">80</option>
          <option value="90">90</option>
          <option value="100">100</option>
        </select>

      </div>
      <canvas id={state.id} width={state.width} height={state.height} />
    </div>
  )
}

export default App
