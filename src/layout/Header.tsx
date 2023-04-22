import Logo from "@src/style/images/logo.png";

export const Header = () => {
  return (
    <header className="border-b border-gray-200 bg-white">
      <nav
        className="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8"
        aria-label="Global"
      >
        <div className="flex lg:flex-1">
          <a href="#" className="-m-1.5 p-1.5">
            <span className="sr-only">BellMemo</span>
            <img className="h-8 w-auto" src={Logo} alt="" />
          </a>
        </div>
        <div className="hidden lg:flex lg:flex-1 lg:justify-end">
          <a href="#" className="text-sm font-semibold leading-6 text-gray-900">
            关于
          </a>
        </div>
      </nav>
    </header>
  );
};
