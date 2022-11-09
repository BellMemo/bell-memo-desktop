import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  Typography,
} from "@mui/material";
import { message } from "@src/components";
import { confirm } from "@src/components/Confirm";
import { useMergedState } from "@src/hooks";
import { useAppDispatch } from "@src/states";
import { reload } from "@src/states/features/recordSlice";
import { Records } from "@src/types";
import { noop } from "@src/util/noop";
import { invoke } from "@tauri-apps/api";
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
  const dispatch = useAppDispatch();

  const handleDelete = async () => {
    const yes = await confirm.error({
      title: `是否确认删除${record.title}？`,
      content: '删除后，相关记录将无法恢复，请谨慎考虑。点击确认即可删除。'
    });
    if (!yes) {
      return;
    }
    const result = await invoke("delete_memo_data", {
      params: { id: record.id },
    });
    if (result) {
      message.success({
        content: "删除成功",
      });
      dispatch(reload());
    }
  };

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
          <Button color="error" variant="outlined" onClick={handleDelete}>
            删除
          </Button>
          <Button color="primary" variant="outlined">
            编辑
          </Button>
        </div>
        <div>
          <Button
            color="info"
            variant="contained"
            onClick={() => setVisible(false)}
          >
            关闭
          </Button>
        </div>
      </DialogActions>
    </Dialog>
  );
};
