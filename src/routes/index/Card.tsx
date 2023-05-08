import classnames from "classnames";
import { EllipsisVerticalIcon } from "@heroicons/react/20/solid";
import { Tag } from "@src/components";
import { Copy } from "@src/components/Copy";

const project = {
  name: "React Components",
  initials: "RC",
  href: "#",
  members: 8,
  bgColor: "bg-green-500",
};

interface CardProps {
  title: React.ReactNode;
  desc: React.ReactNode;
  tags: string[];
}

export const Card: React.FC<CardProps> = (props) => {
  const {title,desc,tags} = props;
  return (
    <div className="flex rounded-md shadow-sm">
      <div
        className={classnames(
          project.bgColor,
          "flex w-16 flex-shrink-0 items-center justify-center rounded-l-md text-sm font-medium text-white"
        )}
      >
        {project.initials}
      </div>
      <div className="flex flex-1 items-center justify-between truncate rounded-r-md border-b border-r border-t border-gray-200 bg-white">
        <div className="flex-1 truncate px-4 py-2 text-sm space-y-2">
          <p className="font-medium text-gray-900 hover:text-gray-600">
            {title}
          </p>
          <div className="flex flex-1 items-center space-x-1">
            <p className="text-gray-500">{desc}</p>
            <Copy text={desc as string} />
          </div>
          <div className="pt-2">
            {
              tags.map((i,index) => {
                return <Tag content={i} key={index} />
              })
            }
          </div>
        </div>
        <div className="flex-shrink-0 pr-2">
          <button
            type="button"
            className="inline-flex h-8 w-8 items-center justify-center rounded-full bg-transparent bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <span className="sr-only">Open options</span>
            <EllipsisVerticalIcon className="h-5 w-5" aria-hidden="true" />
          </button>
        </div>
      </div>
    </div>
  );
};
