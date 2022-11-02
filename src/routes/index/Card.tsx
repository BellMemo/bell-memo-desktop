import { Tag } from "@src/components/Tag";
import { Records } from "@src/types";
import { ContentCopy } from "@mui/icons-material";
import { writeText } from "@tauri-apps/api/clipboard";
import styles from "./Card.module.less";
import { Tooltip } from "@mui/material";
import { message } from "@src/components";

interface CardProps {
  record: Records;
}

export const Card: React.FC<CardProps> = (props) => {
  const { record } = props;

  const handleCopy = async (v) => {
    await writeText(v);
    message.success({ content: "复制成功" });
  };

  return (
    <div className={styles.card}>
      <div className={styles.title}>{record.title}</div>
      <div className={styles.tags}>
        {(record.tags || []).map((i) => {
          return <Tag content={i.name} key={i.id} />;
        })}
      </div>
      <div className={styles.content}>
        <div>{record.content}</div>
        <Tooltip title="点击复制">
          <ContentCopy
            fontSize="small"
            color="action"
            onClick={() => {
              handleCopy(record.content);
            }}
          />
        </Tooltip>
      </div>
    </div>
  );
};
