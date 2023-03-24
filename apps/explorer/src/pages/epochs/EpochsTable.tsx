// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useRpcClient } from '@mysten/core';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { TableFooter } from '~/components/Table/TableFooter';
import { SuiAmount, TxTableCol } from '~/components/transactions/TxCardUtils';
import { TxTimeType } from '~/components/tx-time/TxTimeType';
import {
    CheckpointLink,
    CheckpointSequenceLink,
    EpochLink,
} from '~/ui/InternalLink';
import { usePaginationStack } from '~/ui/Pagination';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';
import { Text } from '~/ui/Text';
import { EpochTimer } from './EpochTimer';
import { EpochProgress } from './stats/EpochProgress';

interface EpochsTableProps {
    initialLimit: number;
    disablePagination?: boolean;
    refetchInterval?: number;
}

export function EpochsTable({
    initialLimit,
    disablePagination,
    refetchInterval,
}: EpochsTableProps) {
    const rpc = useRpcClient();
    const [limit, setLimit] = useState(initialLimit);

    const countQuery = useQuery(
        ['epochs', 'count'],
        async () => (await rpc.getCurrentEpoch()).epoch
    );

    const pagination = usePaginationStack<number>();

    const { data: epochsData } = useQuery(
        ['epochs', { limit, cursor: pagination.cursor }],
        async () => {
            const data = await rpc.getEpochs({
                limit,
                cursor: pagination.cursor,
                descendingOrder: true,
            });
            return data;
        },

        {
            keepPreviousData: true,
            // Disable refetching if not on the first page:
            refetchInterval: pagination.cursor ? undefined : refetchInterval,
        }
    );

    const epochsTable = useMemo(
        () =>
            epochsData
                ? {
                      data: epochsData?.data.map((epoch) => ({
                          epoch: (
                              <TxTableCol isHighlightedOnHover>
                                  <EpochLink epoch={epoch.epoch.toString()} />
                              </TxTableCol>
                          ),
                          transactions: (
                              <TxTableCol>
                                  {epoch.epochTotalTransactions}
                              </TxTableCol>
                          ),
                          stakeRewards: (
                              <TxTableCol>
                                  <SuiAmount
                                      amount={
                                          epoch.endOfEpochInfo
                                              ?.totalStakeRewardsDistributed
                                      }
                                  />
                              </TxTableCol>
                          ),
                          checkpoints: (
                              <TxTableCol>
                                  <CheckpointSequenceLink
                                      sequence={epoch.firstCheckpointId.toString()}
                                  />
                                  {` - `}
                                  <CheckpointSequenceLink
                                      sequence={
                                          epoch.endOfEpochInfo?.lastCheckpointId.toString() ??
                                          ''
                                      }
                                  />
                              </TxTableCol>
                          ),
                          storageRevenue: (
                              <TxTableCol>
                                  {epoch.endOfEpochInfo?.storageCharge}
                              </TxTableCol>
                          ),
                          time: (
                              <TxTableCol>
                                  <TxTimeType
                                      timestamp={
                                          epoch.endOfEpochInfo
                                              ?.epochEndTimestamp
                                      }
                                  />
                              </TxTableCol>
                          ),
                      })),
                      columns: [
                          {
                              header: 'Epoch',
                              accessorKey: 'epoch',
                          },
                          {
                              header: 'Transactions',
                              accessorKey: 'transactions',
                          },
                          {
                              header: 'Stake Rewards',
                              accessorKey: 'stakeRewards',
                          },
                          {
                              header: 'checkpoints',
                              accessorKey: 'checkpoints',
                          },
                          {
                              header: 'Storage Revenue',
                              accessorKey: 'storageRevenue',
                          },
                          {
                              header: 'Time',
                              accessorKey: 'time',
                          },
                      ],
                  }
                : null,
        [epochsData]
    );

    return (
        <div>
            {!disablePagination && (
                <div className="pb-4">
                    <EpochTimer />
                </div>
            )}
            {epochsTable ? (
                <TableCard
                    data={epochsTable.data}
                    columns={epochsTable.columns}
                />
            ) : (
                <PlaceholderTable
                    rowCount={limit}
                    rowHeight="16px"
                    colHeadings={[
                        'Digest',
                        'Sequence Number',
                        'Time',
                        'Transaction Count',
                    ]}
                    colWidths={['100px', '120px', '204px', '90px', '38px']}
                />
            )}
            <div className="py-3">
                <TableFooter
                    label="Epochs"
                    data={epochsData}
                    count={countQuery.data}
                    limit={limit}
                    onLimitChange={setLimit}
                    pagination={pagination}
                    disablePagination={disablePagination}
                />
            </div>
        </div>
    );
}
