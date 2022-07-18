import clsx from 'clsx';
import React, { ComponentPropsWithRef, forwardRef, LegacyRef, Ref, useMemo } from 'react'
import { useFormContext, UseFormRegister } from 'react-hook-form';
import { z } from 'zod';
import { ComponentBase } from '../../@types/utils';
import { schema } from '../organisms/GameMaker';


interface ZodExtendedInputProps<T> {
  type: 'text' | 'number';
  name: keyof z.infer<typeof schema>;
  label?: string;
  disabled?: boolean;
}

export const ZodExtendedInput = <T extends z.infer<typeof schema>, >(
  props: React.PropsWithChildren<ZodExtendedInputProps<ReturnType<UseFormRegister<T>>>>,
) => {
  const {
    type,
    name,
    label,
    disabled,
  } = props;
  const { register, formState } = useFormContext<z.infer<typeof schema>>();

  /**
   * disable判定
   */
  const isDisabled = useMemo(() => disabled, [disabled]);

  /**
   * zodの型定義をstring型に型宣言
   */
  const invalidFeedback = useMemo<string>(() => {
    if (!formState.errors[name]) return '';
    const msg: unknown = formState.errors[name]?.message;
    return msg as string;
  }, [formState.errors]);

  /**
   * エラー判定
   */
  const isInvalid = useMemo(() => {
    if (!formState.errors[name]) return false;
    return !!formState.errors[name]
  }, [formState.errors])

  return (
    <BaseInput
      uiType="zod"
      type={type}
      label={label}
      {...register(
        name,
        {
          valueAsNumber: type === 'number' ? true : false,
        })
      }
      disabled={isDisabled}
      inputClassName={clsx(
        isInvalid ? "!border-red-300 dark:!border-red-500" : "border-blue-500",
      )}
      invalidFeedback={invalidFeedback}
    />
  );
}

type ZodExtended = 'zod';
type NotZodExtended = 'default';

type ZodExtendedProps = React.PropsWithChildren<{
  uiType: 'zod';
}>;

type NotZodExtendedProps = React.PropsWithChildren<{
  uiType: 'default';
  value: string;
}>;

interface CommonBaseInputProps extends ComponentBase {
  type: 'text' | 'number';
  name: string;
  label?: string;
  disabled?: boolean;
  invalidFeedback?: string;
  inputClassName?: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

/**
 * zodの拡張をしているか否かでPropsを制御　
 * ※ zodを使う場合のほうが多いのでdefaultはZodExtended
 */
type BaseInputProps<T = ZodExtended> = T extends NotZodExtended
  ? NotZodExtendedProps & CommonBaseInputProps
  : ZodExtendedProps & CommonBaseInputProps;

/**
 * Zod拡張か否かの判定処理
 * @param uiType 
 * @returns 
 */
const isZodExtended = (props: ZodExtendedProps | NotZodExtendedProps) : props is BaseInputProps<ZodExtended> => {
  return props.uiType === 'zod';
}

/**
 * 汎用Input
 */
export const BaseInput = forwardRef(<T, >(
  props: BaseInputProps<T>,
  ref: LegacyRef<HTMLInputElement>
) => {
  const {
    type,
    name,
    label,
    disabled,
    className,
    inputClassName,
    invalidFeedback,
    onChange,
  } = props;
  
  // TODO: Idは適当
  const id = `${new Date().toDateString()}-${name}`

  // Zod拡張か否かで値が必要かが変わる
  const value = isZodExtended(props) ? undefined : props.value;

  return (
    <div className={clsx(
      "flex flex-col relative",
      className,
    )}>
      {label && <label htmlFor={id}>
          {label}
        </label>
      }
      <input
        id={id}
        ref={ref}
        type={type}
        name={name}
        value={value}
        disabled={disabled}
        onChange={onChange}
        className={clsx(
          "focus:outline-0 focus:border-2 focus:border-emerald-200 dark:focus:border-emerald-400 border border-gray-200 bg-gray-50 dark:bg-gray-600 dark:border-gray-700 rounded-md p-1",
          invalidFeedback ? 'border-2' : '',
          inputClassName,
        )}
      />
      {invalidFeedback &&
        <div className="z-10 px-0.5 py-0.5 text-sm bg-red-200 dark:bg-red-500 absolute -bottom-7  text-red-500 dark:text-red-200 rounded-md">
          {invalidFeedback}
        </div>
      }
    </div>
  );
});