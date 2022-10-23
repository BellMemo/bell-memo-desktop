export type GetPropertyTypeOfArrayObject<T extends unknown, K extends string> = T extends Readonly<
  Array<{ [k in K]: infer R }>
>
  ? R
  : never;