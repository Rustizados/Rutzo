import React, { useEffect, KeyboardEventHandler, MouseEventHandler } from 'react';
import styles from './Modal.module.scss';

interface ModalProps {
  children: React.ReactNode;
  onClose: () => void;
}

function Modal({ children, onClose }: ModalProps) {
  useEffect(() => {
    const handleEscKeyPress = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscKeyPress);

    return () => {
      document.removeEventListener('keydown', handleEscKeyPress);
    };
  }, [onClose]);

  const handleModalClick: MouseEventHandler<HTMLButtonElement> = () => {
    onClose();
  };

  const handleDialogContentClick: MouseEventHandler<HTMLButtonElement> = (event) => {
    event.stopPropagation();
  };

  const handleDialogContentKeyDown: KeyboardEventHandler<HTMLButtonElement> = (
    event
  ) => {
    if (event.key === 'Escape') {
      onClose();
    }
  };

  return (
    <button
      className={styles.modalOverlay}
      type="button" // Agregar el atributo type al botón
      onClick={handleModalClick}
    >
      <button
        className={styles.modalContent}
        tabIndex={-1}
        type='button' // Agregar el atributo type al botón
        onClick={handleDialogContentClick}
        onKeyDown={handleDialogContentKeyDown}
      >
        {children}
      </button>
    </button>
  );
}

export { Modal };
