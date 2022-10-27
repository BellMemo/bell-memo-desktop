// get random
export const getRandom = (min = 0, max = 10) => {
  return Math.round(Math.random() * (max - min)) + min;
};
