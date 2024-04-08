import { ReactComponent as RutzoLogo } from '@/assets/images/logo.svg';
import { SvgLoader } from '../svg-loader/SvgLoader';

function ApiLoader() {
  return (
    <div className="flex items-center justify-center h-screen">
      <div className="flex flex-col text-center">
        <RutzoLogo />
        <SvgLoader />
      </div>
    </div>
  );
}

export { ApiLoader };
