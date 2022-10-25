import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  TextField,
} from "@mui/material";
import { useAppDispatch, useAppSelector } from "@src/states";
import { setVisible } from "@stores/recordSlice";
import { SelectTag } from "../tag/SelectTag";

export const EditRecord: React.FC = () => {
  const { visible } = useAppSelector((store) => store.record);
  const dispatch = useAppDispatch();

  const handleClose = () => {
    dispatch(setVisible(false));
  };

  return (
    <Dialog fullWidth maxWidth="md" open={visible} onClose={handleClose}>
      <DialogTitle>新增记录</DialogTitle>
      <DialogContent>
        <Box
          component="form"
          sx={{
            display: 'flex',
            flexDirection: "column",
            rowGap: 3,
            marginTop: 2
          }}
        >
          <TextField label="标题" size="medium" fullWidth />
          <SelectTag />
          <TextField label="内容" multiline fullWidth minRows={4} />
        </Box>
      </DialogContent>
      <DialogActions>
        <Button variant="contained">确定</Button>
        <Button variant="outlined" onClick={handleClose}>取消</Button>
      </DialogActions>
    </Dialog>
  );
};
