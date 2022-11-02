import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  Typography,
} from "@mui/material";
import { useMergedState } from "@src/hooks";
import { Records } from "@src/types";
import { noop } from "@src/util/noop";
import styles from "./index.module.less";

interface RecordDetailProps {
  record: Records;
  visible?: boolean;
  onVisibleChange?: (v: boolean) => void;
}

export const RecordDetail: React.FC<RecordDetailProps> = (props) => {
  const { record } = props;
  const [visible, setVisible] = useMergedState(false, {
    value: props?.visible,
    onChange: props?.onVisibleChange || noop,
  });
  return (
    <Dialog
      open={visible}
      onClose={() => setVisible(false)}
      fullWidth
      maxWidth="md"
    >
      <DialogTitle>记录详情</DialogTitle>
      <DialogContent className={styles.detail}>
        <div className={styles["detail-item"]}>
          <Typography variant="button" display="block" gutterBottom>
            标题
          </Typography>
          <DialogContentText className="content">
            {record?.title || "-"}
          </DialogContentText>
        </div>
        <div className={styles["detail-item"]}>
          <Typography variant="button" display="block" gutterBottom>
            标签
          </Typography>
          <DialogContentText className="content">
            {record?.title || "-"}
          </DialogContentText>
        </div>
        <div className={styles["detail-item"]}>
          <Typography variant="button" display="block" gutterBottom>
            内容
          </Typography>
          <DialogContentText className="content">
            {record?.content || "-"}
          </DialogContentText>
        </div>
      </DialogContent>
      <DialogActions className={styles.footer}>
        <div>
          <Button color="error" variant="outlined">删除</Button>
          <Button color='primary' variant="outlined">编辑</Button>
        </div>
        <div>
          <Button color="info" variant="contained" onClick={() => setVisible(false)}>关闭</Button>
        </div>
      </DialogActions>
    </Dialog>
  );
};
