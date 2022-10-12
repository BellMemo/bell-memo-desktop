import { Outlet } from "react-router-dom";

export const EmptyLayout: React.FC = (props) => {
  console.log(props);
  return (
    <div>
      <Outlet />
    </div>
  );
};
