import React, { useCallback } from 'react'
import { useFieldArray, useForm, useFormContext } from 'react-hook-form';
import z from 'zod';
import { EV } from '~/@types/utils';
import { parseIntOr, unwrapEV } from './../../assets/utils/common';
import { schema } from '../organisms/GameMaker';

interface BaseCheckProps {
  id: string;
  label: string;
  value: string | number;
  checked: boolean;
  onChange: (ev: EV<HTMLInputElement>) => void;
}

export const BaseCheck: React.FC<BaseCheckProps> = ({
  id,
  label,
  value,
  checked,
  onChange,
}) => {
  return (
    <div className="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
      {/* TODO: なぜかチェックの挙動がおかしい */}
      <input
        id={id}
        type="checkbox"
        value={value}
        checked={checked}
        onChange={onChange}
        className="w-4 h-4 text-blue-600 bg-gray-100 rounded border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
      />
      <label
        htmlFor={id}
        className="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300"
      >{label}</label>
    </div>
  ) 
}

interface ScrollCheckGroupProps {

}

const SHOT_BEHAVIOR_OPTIONS = {
  NORMAL: {
    LABEL: 'デフォルト',
    VALUE: 0,
  },
  SPEED_UP: {
    LABEL: '加速',
    VALUE: 1,
  },
  SPEED_DOWN: {
    LABEL: '減速',
    VALUE: 2,
  },
  REFLECT: {
    LABEL: '反射',
    VALUE: 3,
  },
  RANDOM: {
    LABEL: 'ランダム',
    VALUE: 4,
  },
  SLEEP: {
    LABEL: 'スリープ',
    VALUE: 5,
  },
};
const shotBehaviorOptions = Object.entries(SHOT_BEHAVIOR_OPTIONS)
  .map(([key, schema]) => {
    return {
      key,
      label: schema.LABEL,
      value: schema.VALUE,
    }
  })

export const ScrollCheckGroup: React.FC<ScrollCheckGroupProps> = ({}) => {
    const context = useFormContext()

    const checked = context.getValues('shot_behavior') || [];

    const handleChange = (ev: EV<HTMLInputElement>) => {
      const value = unwrapEV(ev);
      const intVal = parseIntOr(value);
      const oldValues = context.getValues('shot_behavior');
      const newValues = oldValues.includes(intVal)
        ? oldValues.filter((v: number) => v !== intVal)
        : oldValues.concat([intVal]);
      context.setValue('shot_behavior', newValues, {
        shouldDirty: true,
        shouldValidate: true,
      })
    }

    return (
      <div className="bg-white rounded shadow dark:bg-gray-700">
        <label>弾の挙動</label>
        <ul className="overflow-y-auto px-3 pb-3 h-48 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">
          {shotBehaviorOptions.map((option) => (
            <li key={option.key}>
              <BaseCheck
                id={option.key}
                checked={checked.includes(option.value)}
                value={option.value}
                label={option.label}
                onChange={handleChange}
              />
            </li>
          ))}
        </ul>
      </div>
    );
}