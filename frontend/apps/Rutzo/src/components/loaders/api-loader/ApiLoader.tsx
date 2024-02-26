// import { Logo } from '@/components/layout/header/logo/logo';
import { SvgLoader } from '../svg-loader/SvgLoader'; 

function ApiLoader() {
  return (
    <div className="flex flex-col items-center justify-center h-full">
      {/*<Logo />*/}
      <SvgLoader />
    </div>
  );
}

export { ApiLoader };
