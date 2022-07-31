import { useAtom } from 'jotai';
import React, { useCallback, useEffect, useMemo, useRef } from 'react'
import { FormProvider, useForm, useFormContext, UseFormRegister } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { gameStateAtom } from '../../stores/gameStore';
import { Canvas, CanvasHandler } from './Canvas';
import clsx from 'clsx';
import { ZodExtendedInput } from '../atoms/BaseInput';
import { zodNumber } from '../../assets/utils/validation';
import { ComponentBase } from '~/@types/utils';
import { Container } from '../atoms/Container';
import { BaseBtn, ButtonBtn } from '../atoms/BaseBtn';
import { StartIcon, StopIcon, TmpStopIcon } from '../atoms/Icons';
import { BtnGroup } from '../molecules/BtnGroup';
import { Panel } from '../atoms/Panel';
import { Schedule } from './Schedule';
import { ScrollCheckGroup } from '../molecules/ScrollCheckGroup';

// interface ConnectFormProps<T> {
//   children: React.FC<{
//     register: UseFormRegister<T>,
//   }>;
// }

// export const ConnectForm = <T, >(props: React.PropsWithChildren<ConnectFormProps<T>>) => {
//   const { children } = props;
//   const methods = useFormContext<T>();
//   return children({ ...methods })
// }

// const TextComponent: React.FC<{}> = ({ }) => {
//   <ConnectForm<{register: UseFormRegister<{ shotSpeed: number; }>}>>
//     {({ register }) => <input {...register("shotSpeed")} />}
//   </ConnectForm>
// }


export const schema = z.object({
  disk_size: zodNumber({ min: 1, max: 100 }),
  shot_way_num: zodNumber({ min: 1, max: 100 }),
  shot_speed: zodNumber({}),
  shot_interval: zodNumber({ min: 50, max: 1000 }),
  shot_behavior: z.array(zodNumber({ min: 0, max: 6 })),
  speed_change_per: zodNumber({ min: 0, max: 100 }),
  speed_change_interval: zodNumber({ min: 10, max: 100 }),
  x_coordinate: zodNumber({ min: 1, max: 450 }), // TODO: ストアの値でルールを更新
  y_coordinate: zodNumber({ min: 1, max: 800 }), // TODO: ストアの値でルールを更新
  reflect_count: zodNumber({ min: 0, max: 3 }), // TODO: ストアの値でルールを更新
  start_at: zodNumber({ min: 0, max: 10000 }), // TODO: ストアの値でルールを更新
  end_at: zodNumber({ min: 0, max: 10000 }), // TODO: ストアの値でルールを更新
  sleep_interval: zodNumber({ min: 0, max: 1000 }), // TODO: ストアの値でルールを更新
  sleep_timeout: zodNumber({ min: 0, max: 1000 }), // TODO: ストアの値でルールを更新
  degree_change_by: zodNumber({ min: -360, max: 360 }), // TODO: ストアの値でルールを更新
  disk_type: zodNumber({ min: 0, max: 4 }), // TODO: ストアの値でルールを更新
  disk_color: zodNumber({ min: 0, max: 8 }), // TODO: ストアの値でルールを更新
  gravity_direction: zodNumber({ min: 0, max: 3 }), // TODO: ストアの値でルールを更新
  gravity_change_per: zodNumber({ min: 0, max: 100 }), // TODO: ストアの値でルールを更新
})
.refine((values) => {
  if (values.sleep_interval <= values.sleep_timeout) {
    return false;
  }
  if (values.start_at >= values.end_at) {
    return false;
  }
  return true;
})

interface GameMakerProps extends ComponentBase {
  
}

export const GameMaker: React.FC<GameMakerProps> = (props) => {
  const { className } = props;
  const canvasRef = useRef({} as CanvasHandler);
  const [canvasState, setCanvasState] = useAtom(gameStateAtom);

  /** zod */
  const methods = useForm<z.infer<typeof schema>>({
    defaultValues: {
      disk_size: 1,
      shot_way_num: 10,
      shot_speed: 1,
    },
    mode: 'onChange',
    reValidateMode: 'onChange',
    resolver: zodResolver(schema),
  })

  /** onChangeで発火 */
  const handleSubmit = methods.handleSubmit((data) => {
    setCanvasState((old) => {
      return {
        ...old,
        ...data,
      } 
    });
    // HACK: 別スレッドで実行 stateの更新後に実行する
    setTimeout(() => {
      canvasRef.current.updateCanvasState();
    })
  });

  /** 再生 */
  const gameStart = useCallback(() => {
    canvasRef.current.gameStart();
  }, [])

  /** 一時停止 */
  const gameTmpStop = useCallback(() => {
    canvasRef.current.gameTmpStop();
  }, [])

  /** 停止 */
  const gameStop = useCallback(() => {
    canvasRef.current.gameStop();
  }, [])

  /** 初期化処理 */
  useEffect(() => {
    methods.reset({
      ...canvasState,
    });
  }, [])

  const handleShotTypeChange = useCallback((shotType: number) => {
    setCanvasState((old) => ({ ...old, shot_type: shotType }));
    setTimeout(() => {
      canvasRef.current.updateCanvasState();
    })
  }, [])

  const screenBtn = useMemo(() => {
    return [
      {
        icon: <StopIcon />,
        onClick: gameStop,
      },
      {
        icon: <TmpStopIcon />,
        onClick: gameTmpStop,
      },
      {
        icon: <StartIcon />,
        onClick: gameStart, 
      },
    ]
  }, [])

  const shotTypeBtn = useMemo(() => {
    return [
      {
        icon: <span>ランダム</span>,
        onClick: () => handleShotTypeChange(0),
      },
      {
        icon: <span>放射状</span>,
        onClick: () => handleShotTypeChange(1),
      },
      {
        icon: <span>放射状回転</span>,
        onClick: () => handleShotTypeChange(2),
      },
      {
        icon: <span>渦巻状</span>,
        onClick: () => handleShotTypeChange(3),
      },
      {
        icon: <span>撃ち降ろし</span>,
        onClick: () => handleShotTypeChange(4),
      },
    ]
  }, [])

  return (
    <div className={clsx(
      "flex gap-4 h-[calc(100vh_-_84px)]",
      className,
    )}>
      <Panel className="flex-1 flex overflow-y-scroll">
        <>
          <div className="mx-2">
            <BtnGroup
              direction="vertical"
              className="flex flex-col"
              btnClass="px-4 py-3"
              schema={shotTypeBtn}
            ></BtnGroup>
          </div>
          <FormProvider {...methods}>
            <form onChange={handleSubmit} className="y-interval flex-1">

        {/* {JSON.stringify(methods.getValues())} */}
              <div className="bg-gray-100 dark:bg-gray-800 p-2 rounded-md text-gray-400 dark:text-gray-400">
                <p>ディスク設定</p>
              </div>
              
              <ZodExtendedInput
                label="ディスクサイズ"
                type='number'
                name='disk_size'
              />

              <select {...methods.register('disk_type', {
                valueAsNumber: true,
              })}
              className={clsx("focus:outline-0 focus:border-2 focus:border-emerald-200 dark:focus:border-emerald-400 border border-gray-200 bg-gray-50 dark:bg-gray-600 dark:border-gray-700 rounded-md p-1")}
              >
                <option value="0">小丸弾</option>
                <option value="1">中丸弾</option>
                <option value="2">中丸弾2</option>
                <option value="3">中丸弾3</option>
                <option value="4">大弾</option>
              </select>

              <select {...methods.register('disk_color', {
                valueAsNumber: true,
              })}
              className={clsx("focus:outline-0 focus:border-2 focus:border-emerald-200 dark:focus:border-emerald-400 border border-gray-200 bg-gray-50 dark:bg-gray-600 dark:border-gray-700 rounded-md p-1")}
              >
                <option value="0">赤</option>
                <option value="1">オレンジ</option>
                <option value="2">黄</option>
                <option value="3">緑</option>
                <option value="4">水色</option>
                <option value="5">青</option>
                <option value="6">紺</option>
                <option value="7">紫</option>
                <option value="8">ピンク</option>
              </select>

              <ZodExtendedInput
                label="重力速度変化率(%)"
                type='number'
                name='gravity_change_per'
              />

              <select {...methods.register('gravity_direction', {
                valueAsNumber: true,
              })}
              className={clsx("focus:outline-0 focus:border-2 focus:border-emerald-200 dark:focus:border-emerald-400 border border-gray-200 bg-gray-50 dark:bg-gray-600 dark:border-gray-700 rounded-md p-1")}
              >
                <option value="0">下</option>
                <option value="1">右</option>
                <option value="2">左</option>
                <option value="3">上</option>
              </select>


              <ScrollCheckGroup />

              <div className="bg-gray-100 dark:bg-gray-800 p-2 rounded-md text-gray-400 dark:text-gray-400">
                <p>スケジュール設定(仮)</p>
              </div>

              <ZodExtendedInput
                label="ショットWAY数"
                type='number'
                name='shot_way_num'
              />

              <ZodExtendedInput
                label="ショット速度"
                type='number'
                name='shot_speed'
              />

              <ZodExtendedInput
                label="ショット間隔"
                type='number'
                name='shot_interval'
              />

  

              <ZodExtendedInput
                label="速度変化率"
                type='number'
                name='speed_change_per'
              />

              <ZodExtendedInput
                label="速度変化間隔"
                type='number'
                name='speed_change_interval'
              />

              <ZodExtendedInput
                label="反射回数"
                type='number'
                name='reflect_count'
              />

              <ZodExtendedInput
                label="射出角変化量(°)"
                type='number'
                name='degree_change_by'
              />

              <div className="flex flex-col y-interval">
                <span>スリープ</span>
                <div className="flex gap-4">
                  <ZodExtendedInput
                    label="インターバルms"
                    type='number'
                    name='sleep_interval'
                  />
                  <ZodExtendedInput
                    label="スリープms"
                    type='number'
                    name='sleep_timeout'
                  />
                </div>
              </div>

              <div className="flex flex-col y-interval">
                <span>射出位置</span>
                <div className="flex gap-4">
                  <ZodExtendedInput
                    label="X座標"
                    type='number'
                    name='x_coordinate'
                  />
                  <ZodExtendedInput
                    label="Y座標"
                    type='number'
                    name='y_coordinate'
                  />
                </div>
              </div>

              <div className="flex flex-col y-interval">
                <span>スケジュール</span>
                <div className="flex gap-4">
                  <ZodExtendedInput
                    label="開始ms"
                    type='number'
                    name='start_at'
                  />
                  <ZodExtendedInput
                    label="終了ms"
                    type='number'
                    name='end_at'
                  />
                </div>
              </div>

              {/* <select {...methods.register('shot_speed', {
                valueAsNumber: true,
              })}>
                <option value="1">1</option>
                <option value="2">2</option>
                <option value="3">3</option>
                <option value="4">4</option>
                <option value="5">5</option>
            </select>*/}

              

              <div className="mt-auto">
                <BaseBtn
                  type="submit"
                  className={clsx(
                    "text-white bg-zinc-400",
                    methods.formState.isValid ? "bg-blue-600" : "bg-red-600"
                  )}
                >保存</BaseBtn>
              </div>

              <Schedule />

            </form>
          </FormProvider>

        </>
      </Panel>
      {/* <Container className="flex flex-col"> */}
        <Panel>
          <>
            <div className="flex">
              <BtnGroup direction="horizontal" schema={screenBtn} />
            </div>
            <Canvas ref={canvasRef} />
          </>
        </Panel>
      {/* </Container> */}
    </div>
  );
}