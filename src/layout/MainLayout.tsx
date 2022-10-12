import { Outlet } from "react-router-dom";
import styles from './main.module.less';

export const MainLayout: React.FC = () => {
  return (
    <div className={styles.main}>
      <Outlet />
    </div>
  );
};
