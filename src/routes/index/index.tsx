import { useEffect, useState } from "react";
import { Card } from "./Card";
import { request } from "@src/util/request";

export const Index = () => {
  const [data, setData] = useState([]);

  const fetchData = async () => {
    const resp = await request("select_memo_data", { params: { content: "" } });
    setData(resp);
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div className="mt-3 grid grid-cols-1 gap-5 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4">
      {data.map((i) => {
        return (
          <Card
            title={i.title}
            desc={i.content}
            tags={(i?.tags || []).map((i) => i.name)}
          />
        );
      })}
    </div>
  );
};
