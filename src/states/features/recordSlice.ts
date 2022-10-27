import { createSlice } from "@reduxjs/toolkit";

export interface RecordState {
  visible: boolean;
  reloadState: any;
}
const initialState: RecordState = {
  visible: false,
  reloadState: {},
};

export const recordSlice = createSlice({
  name: "record",
  initialState,
  reducers: {
    setVisible: (state, { payload }) => {
      state.visible = payload;
    },
    reload: (state) => {
      state.reloadState = {};
    },
  },
});

export const { setVisible, reload } = recordSlice.actions;
export default recordSlice.reducer;
