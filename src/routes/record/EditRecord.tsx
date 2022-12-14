import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  TextField,
} from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { MemoTag } from "@src/types";
import { noop } from "@src/util/noop";
import { SelectTag } from "../tag/SelectTag";

interface EditRecordProps {
  record?: {
    id: string;
    title: string;
    tags: Partial<MemoTag>[];
    content: string;
  };
  visible?: boolean;
  setVisible?: (v: boolean) => void;
  onClose?: () => void;
}

export const EditRecord: React.FC<EditRecordProps> = (props) => {
  const { record, visible, setVisible, onClose = noop } = props;

  const [value, setValue] = useState({
    title: "",
    tags: [],
    content: "",
  });

  const handleClose = () => {
    setValue({
      title: "",
      tags: [],
      content: "",
    });
    setVisible(false);
  };

  const handleSubmit = async () => {
    const result = await invoke("edit_memo_data", {
      params: {
        ...value,
        tags: value.tags.map((i) => i.id),
      },
    });
    if (result) {
      handleClose();
      onClose();
    }
  };

  useEffect(() => {
    if (record && visible) {
      setValue(record);
    }
  }, [record, visible]);

  return (
    <Dialog fullWidth maxWidth="md" open={visible} onClose={handleClose}>
      <DialogTitle>编辑记录</DialogTitle>
      <DialogContent>
        <Box
          component="form"
          sx={{
            display: "flex",
            flexDirection: "column",
            rowGap: 3,
            marginTop: 2,
          }}
        >
          <TextField
            label="标题"
            size="medium"
            fullWidth
            value={value.title}
            onChange={(e) => {
              setValue({
                ...value,
                title: e.target.value,
              });
            }}
          />
          <SelectTag
            value={value.tags}
            onChange={(v) =>
              setValue({
                ...value,
                tags: v,
              })
            }
          />
          <TextField
            label="内容"
            multiline
            fullWidth
            minRows={4}
            value={value.content}
            onChange={(e) => {
              setValue({
                ...value,
                content: e.target.value,
              });
            }}
          />
        </Box>
      </DialogContent>
      <DialogActions>
        <Button variant="contained" onClick={handleSubmit}>
          确定
        </Button>
        <Button variant="outlined" onClick={handleClose}>
          取消
        </Button>
      </DialogActions>
    </Dialog>
  );
};
