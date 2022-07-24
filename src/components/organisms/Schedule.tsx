import { useAtom } from "jotai";
import React, { Component, useEffect, useRef } from "react";
import { threadsStateAtom } from "../../stores/threadStore";

interface ScheduleProps {

}

export const Schedule: React.FC<ScheduleProps> = ({}) => {
  // const tasks = [
  //   {
  //     id: "Task 1",
  //     name: "Redesign website",
  //     start: "2016-12-28",
  //     end: "2016-12-31",
  //     progress: 10,
  //     dependencies: "",
  //   },
  //   {
  //     id: "Task 2",
  //     name: "Redesign website",
  //     start: "2016-12-28",
  //     end: "2016-12-31",
  //     progress: 20,
  //     dependencies: "Task 1",
  //   },
  //   {
  //     id: "Task 3",
  //     name: "Redesign website",
  //     start: "2016-12-28",
  //     end: "2016-12-31",
  //     progress: 0,
  //     dependencies: "Task 2, Task 1",
  //   },
  // ].map((x) => new Task(x));

  // const []
  const [threadsState, setThreadsState] = useAtom(threadsStateAtom)
  // useEffect(() => {
    
  // }, [])
  // const addThread = () => {
  //   setThreadsState((old) => {
  //     return [...old, null]
  //   })
  // }

    return (
      <div className="flex flex-col border border-gray-500 rounded-md p-2">
        {/* <button onClick={addThread}>add thread</button> */}
        {/* {schedules.map((schedule) => (
          <div key={schedule.id}>

          </div>
        ))}
        // <button onClick={addSchedule}>add schedule</button> */}
        {/* <div className="flex">
          {Array(24).fill('').map((_, i) => (
            <div key={i}>{i}</div>
          ))}
        </div>

        <svg className="h-64 w-full text-gray-500" 
          width="100"
          height="100"
          viewBox="0 0 100 100"
          stroke-width="0.1"
          stroke="currentColor"
          fill="none"
          stroke-linecap="round"
          stroke-linejoin="round"
        > 
          <g>
            <line x1="0" y1="4"  x2="24" y2="4" />
            <line x1="0" y1="6"  x2="24" y2="6" />
            <line x1="0" y1="8"  x2="24" y2="8" />
            <line x1="0" y1="10" x2="24" y2="10" />
            <line x1="0" y1="12" x2="24" y2="12" />
            <line x1="0" y1="14" x2="24" y2="14" />
          </g>
        </svg> */}
      </div>
    );
}