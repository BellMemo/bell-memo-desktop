import { getRandom } from "@src/util/utils";
import styles from "./Tag.module.less";

interface TagProps {
  content: string;
}

const TagTheme = [
  "#4C6BA1",
  "#D9CBBE",
  "#409544",
  "#6DA78F",
  "#A62626",
  "#BF7DFA",
  "#7472D6",
  "#85B4EB",
  "#81D2D6",
  "#81F7BE",
];

export const Tag: React.FC<TagProps> = (props) => {
  const theme = getRandom(0, 9);
  const { content } = props;
  return (
    <div className={styles.tag} style={{ backgroundColor: TagTheme[theme] }}>
      {content}
    </div>
  );
};
