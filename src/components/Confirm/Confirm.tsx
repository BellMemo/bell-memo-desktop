import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
} from "@mui/material";
import { noop } from "@src/util/noop";
import { useState } from "react";
import { red, green, blue, orange } from "@mui/material/colors";
import { ErrorOutline, InfoOutlined } from "@mui/icons-material";
import styles from "./Confirm.module.less";

const Icon = {
  danger: <ErrorOutline fontSize="medium" sx={{ color: red["500"] }} />,
  warning: <InfoOutlined fontSize="medium" sx={{ color: orange["500"] }} />,
  success: <InfoOutlined fontSize="medium" sx={{ color: green["500"] }} />,
  default: <InfoOutlined fontSize="medium" sx={{ color: blue["500"] }} />,
};

type ConfirmTheme = "default" | "danger" | "warning" | "none" | "success";

export interface ConfirmProps {
  title?: React.ReactNode;
  content: React.ReactNode;
  okText?: React.ReactNode;
  cancelText?: React.ReactNode;
  type?: ConfirmTheme;
  onOk?: () => void;
  onCancel?: () => void;
  defaultVisible?: boolean;
  icon?: React.ReactNode;
}

export const Confirm: React.FC<ConfirmProps> = (props) => {
  const {
    title,
    content,
    okText = "确定",
    cancelText = "取消",
    type = "default",
    defaultVisible = false,
    onOk = noop,
    onCancel = noop,
    icon,
  } = props;

  const [visible, setVisible] = useState(defaultVisible);

  const handleClose = () => {
    setVisible(false);
  };

  const handleOk = () => {
    handleClose();
    onOk();
  };

  const handleCancel = () => {
    handleClose();
    onCancel();
  };

  return (
    <Dialog
      open={visible}
      onClose={() => setVisible(false)}
      fullWidth
      maxWidth="sm"
    >
      <DialogContent className={styles.content}>
        {!!title && <div className={styles.title}>{title}</div>}
        <div className={styles.icon}>{icon || Icon[type]}</div>
        <div className={styles.content}>{content}</div>
      </DialogContent>
      <DialogActions className={styles.footer}>
        <Button onClick={handleOk} variant="contained">
          {okText}
        </Button>
        <Button onClick={handleCancel} variant="outlined">
          {cancelText}
        </Button>
      </DialogActions>
    </Dialog>
  );
};
