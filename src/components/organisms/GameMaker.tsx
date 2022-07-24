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
  disk_size: zodNumber({ min: 1, max: 10 }),
  // shot_type: zodNumber({ min: 0, max: 3 }),
  shot_way_num: zodNumber({ min: 1, max: 100 }),
  shot_speed: zodNumber({}),
  shot_interval: zodNumber({ min: 100, max: 1000 }),
  shot_behavior: zodNumber({ min: 0, max: 5 }),
  speed_change_per: zodNumber({ min: 0.1, max: 5 }),
  speed_change_interval: zodNumber({ min: 10, max: 100 }),
  x_coordinate: zodNumber({ min: 1, max: 450 }), // TODO: ストアの値でルールを更新
  y_coordinate: zodNumber({ min: 1, max: 800 }), // TODO: ストアの値でルールを更新
  reflect_count: zodNumber({ min: 0, max: 3 }), // TODO: ストアの値でルールを更新
  start_at: zodNumber({ min: 0, max: 10000 }), // TODO: ストアの値でルールを更新
  end_at: zodNumber({ min: 0, max: 10000 }), // TODO: ストアの値でルールを更新
  sleep_interval: zodNumber({ min: 0, max: 1000 }), // TODO: ストアの値でルールを更新
  sleep_timeout: zodNumber({ min: 0, max: 1000 }), // TODO: ストアの値でルールを更新
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
      "flex gap-4",
      className,
    )}>
      <Panel className="flex-1 flex">
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

              <ZodExtendedInput
                label="ディスクサイズ"
                type='number'
                name='disk_size'
              />

              {/* <ZodExtendedInput
                label="ショット種別"
                type='number'
                name='shot_type'
              /> */}

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
                label="弾の挙動"
                type='number'
                name='shot_behavior'
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
              </select>

              <select {...methods.register('shot_speed', {
                valueAsNumber: true,
              })}>
                <option value="1">1</option>
                <option value="2">2</option>
                <option value="3">3</option>
                <option value="4">4</option>
                <option value="5">5</option>
              </select> */}

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