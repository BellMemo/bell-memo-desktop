import { Tag, message } from "@src/components";
import { Records } from "@src/types";
import { ContentCopy } from "@mui/icons-material";
import { writeText } from "@tauri-apps/api/clipboard";
import { Tooltip } from "@mui/material";
import styles from "./Card.module.less";
import { RecordDetail } from "../detail";
import { useState } from "react";

interface CardProps {
  record: Records;
}

export const Card: React.FC<CardProps> = (props) => {
  const { record } = props;
  const [detailVisible, setDetailVisible] = useState(false);

  const handleCopy = async (v) => {
    await writeText(v);
    message.success({ content: "复制成功" });
  };

  return (
    <>
      <div className={styles.card} onClick={() => setDetailVisible(true)}>
        <div className={styles.title}>{record.title}</div>
        <div className={styles.tags}>
          {(record.tags || []).map((i) => {
            return <Tag content={i.name} key={i.id} />;
          })}
        </div>
        <div className={styles.content}>
          <Tooltip title={record.content}>
            <div>{record.content}</div>
          </Tooltip>
          <Tooltip title="点击复制">
            <ContentCopy
              fontSize="small"
              color="action"
              onClick={(e) => {
                e.stopPropagation();
                handleCopy(record.content);
              }}
            />
          </Tooltip>
        </div>
      </div>
      <RecordDetail
        record={record}
        visible={detailVisible}
        onVisibleChange={setDetailVisible}
      />
    </>
  );
};
