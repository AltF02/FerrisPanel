import React from 'react';
import { Card, CardBody } from '@windmill/react-ui';
import { useHistory } from 'react-router';
import RoundIcon from '../RoundIcon';
import { ServerIcon } from '../../icons';

interface IProps {
  id: string,
  name: string,
  description: string,
}

export default function ServerCard(props: IProps) {
  const { id, name, description } = props;
  const history = useHistory();

  function navigateToServer() {
    history.push(`app/server/${id}`);
  }

  return (
    <Card className="mt-4">
      <CardBody onClick={navigateToServer} className="flex items-center cursor-pointer">
        <RoundIcon
          icon={ServerIcon}
          iconColorClass="text-grey-500 dark:text-grey-100"
          bgColorClass="bg-gray-200 dark:bg-gray-300"
          className="mr-4"
        />
        <div>
          <p className="mb-2 text-lg  font-semibold text-gray-700 dark:text-gray-200">{name}</p>
          <p className="text-sm font-medium text-gray-600 dark:text-gray-400">{description}</p>
        </div>
      </CardBody>
    </Card>
  );
}
