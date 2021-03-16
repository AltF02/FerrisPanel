import React from 'react';
import classNames from 'classnames';

interface IProps {
  icon: any,
  iconColorClass: string,
  bgColorClass: string,
  className: string
}

export default function RoundIcon({
  icon: Icon,
  iconColorClass = 'text-purple-600 dark:text-purple-100',
  bgColorClass = 'bg-purple-100 dark:bg-purple-600',
  className,
}: IProps) {
  const baseStyle = 'p-3 rounded-full';

  const cls = classNames(baseStyle, iconColorClass, bgColorClass, className);

  return (
    <div className={cls}>
      <Icon className="w-5 h-5" />
    </div>
  );
}
