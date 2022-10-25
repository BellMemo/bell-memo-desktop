import { createSlice } from "@reduxjs/toolkit";

export interface RecordState {
  visible: boolean;
}
const initialState: RecordState = {
  visible: false,
};

export const recordSlice = createSlice({
  name: "record",
  initialState,
  reducers: {
    setVisible: (state,{payload}) => {
      state.visible = payload;
    },
  },
});

export const { setVisible } = recordSlice.actions;
export default recordSlice.reducer;
