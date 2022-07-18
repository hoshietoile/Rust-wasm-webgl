import { z } from "zod"

/**
 * 数値ハンドリング用Zod関数
 */
type CustomZodNumber = {
  min?: number;
  max?: number;
}
export const zodNumber = (props: CustomZodNumber) => {
  const { min, max } = props;
  let base = z.number();
  if (min !== undefined) {
    base = base.min(min, {
      message: `数値は${min}以上で入力してください`,
    });
  }
  if (max !== undefined) {
    base = base.max(max, {
      message: `数値は${max}以下で入力してください`,
    });
  }
  return base;
}