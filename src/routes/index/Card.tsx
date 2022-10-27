import { Tag } from "@src/components/Tag";
import { Records } from "@src/types";
import styles from "./Card.module.less";

interface CardProps {
  record: Records;
}

export const Card: React.FC<CardProps> = (props) => {
  const { record } = props;
  return (
    <div className={styles.card}>
      <div className={styles.title}>{record.title}</div>
      <div className={styles.tags}>
        {(record.tags || []).map((i) => {
          return <Tag content={i.name} key={i.id} />;
        })}
      </div>
      <div className={styles.content}>{record.content}</div>
    </div>
  );
};
