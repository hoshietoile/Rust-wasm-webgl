import clsx from 'clsx';
import React, { useCallback, useMemo } from 'react'
import { ComponentBase } from '~/@types/utils';
import { BaseBtn, ButtonBtn } from '../atoms/BaseBtn';

interface BtnGroupElementProps extends ComponentBase {
  idx: number;
  icon: React.ReactChild;
  onClick: (...arg: any[]) => void;
}

export const BtnGroupElement: React.FC<BtnGroupElementProps> = (props) => {
  const { icon, className, onClick } = props;

  return <BaseBtn<ButtonBtn>
    type="button"
    flat={true}
    className={clsx(
      "focus:bg-gray-100 dark:focus:bg-gray-800",
      className,
    )}
    onClick={onClick}
  >
    {icon}
  </BaseBtn> 
}

interface BtnGroupProps extends ComponentBase {
  direction: 'horizontal' | 'vertical';
  btnClass?: string;
  schema: Omit<BtnGroupElementProps, 'idx'>[];
}

export const BtnGroup: React.FC<BtnGroupProps> = (props) => {
  const { direction, schema, btnClass, className } = props;

  const schemaLength = schema.length;

  return (
    <div className={clsx(
      "shadow-lg",
      className,
    )}>
      {schema.map((element, idx) => {
        const directionClass = (() => {
          switch (direction) {
            case 'horizontal':
              return idx === 0
                ? "rounded-br-none rounded-tr-none"
                : idx === schemaLength - 1
                  ? "rounded-bl-none rounded-tl-none"
                  : "rounded-none border-l border-r border-gray-200 dark:border-gray-500";
            case 'vertical':
              return idx === 0
                  ? "rounded-bl-none rounded-br-none border-b"
                  : idx === schemaLength - 1
                    ? "rounded-tl-none rounded-tr-none"
                    : "rounded-none border-b border-gray-200 dark:border-gray-500";
          }
        })();

        return <BtnGroupElement
          key={idx}
          idx={idx}
          className={clsx(
            "",
            btnClass,
            directionClass,
          )}
          icon={element.icon}
          onClick={element.onClick}
        ></BtnGroupElement>
      })}
    </div>
  );
}