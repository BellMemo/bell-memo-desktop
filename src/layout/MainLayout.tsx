import { Outlet } from "react-router-dom";
import { Header } from "./Header";

export const MainLayout = () => {
  return (
    <div className="min-h-full bg-gray-100">
      <Header />
      <div className="py-10">
        <Outlet />
      </div>
    </div>
  );
};
