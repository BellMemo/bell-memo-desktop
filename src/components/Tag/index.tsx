interface TagProps {
  content?: React.ReactNode;
  children?: React.ReactNode;
}

export const Tag: React.FC<TagProps> = (props) => {
  const { content, children } = props;
  return (
    <span className="inline-flex items-center rounded-md bg-yellow-50 px-1.5 py-0.5 text-xs font-medium text-yellow-800 ring-1 ring-inset ring-yellow-600/20">
      {children || content}
    </span>
  );
};
