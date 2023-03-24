// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useRpcClient } from '@mysten/core';
import { useQuery } from '@tanstack/react-query';
import { useEffect, useMemo, useState } from 'react';

import { TableFooter } from '~/components/Table/TableFooter';
import { TxTableCol } from '~/components/transactions/TxCardUtils';
import { TxTimeType } from '~/components/tx-time/TxTimeType';
import { CheckpointLink } from '~/ui/InternalLink';
import { usePaginationStack } from '~/ui/Pagination';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';
import { Text } from '~/ui/Text';

interface CheckpointsTableProps {
    initialCursor?: number;
    initialLimit: number;
    disablePagination?: boolean;
    refetchInterval?: number;
}

export function CheckpointsTable({
    initialCursor,
    initialLimit,
    disablePagination,
    refetchInterval,
}: CheckpointsTableProps) {
    const rpc = useRpcClient();
    const [limit, setLimit] = useState(initialLimit);
    const [cursor, setCursor] = useState(initialCursor);

    const countQuery = useQuery(['checkpoints', 'count'], () =>
        rpc.getLatestCheckpointSequenceNumber()
    );

    const pagination = usePaginationStack<number>();

    console.log(pagination);

    useEffect(() => {
        setCursor(pagination.cursor);
    }, [pagination.cursor]);

    const { data: checkpointsData } = useQuery(
        ['checkpoints', { limit, cursor }],
        () =>
            rpc.getCheckpoints({
                limit,
                descendingOrder: true,
                cursor: cursor,
            }),
        {
            keepPreviousData: true,
            // Disable refetching if not on the first page:
            refetchInterval: pagination.cursor ? undefined : refetchInterval,
        }
    );

    const checkpointsTable = useMemo(
        () =>
            checkpointsData
                ? {
                      data: checkpointsData?.data.map((checkpoint) => ({
                          digest: (
                              <TxTableCol isHighlightedOnHover>
                                  <CheckpointLink digest={checkpoint.digest} />
                              </TxTableCol>
                          ),
                          time: (
                              <TxTableCol>
                                  <TxTimeType
                                      timestamp={checkpoint.timestampMs}
                                  />
                              </TxTableCol>
                          ),
                          sequenceNumber: (
                              <TxTableCol>
                                  <Text
                                      variant="bodySmall/medium"
                                      color="steel-darker"
                                  >
                                      {checkpoint.sequenceNumber}
                                  </Text>
                              </TxTableCol>
                          ),
                          transactionCount: (
                              <TxTableCol>
                                  <Text
                                      variant="bodySmall/medium"
                                      color="steel-darker"
                                  >
                                      {checkpoint.transactions.length}
                                  </Text>
                              </TxTableCol>
                          ),
                      })),
                      columns: [
                          {
                              header: 'Digest',
                              accessorKey: 'digest',
                          },
                          {
                              header: 'Sequence Number',
                              accessorKey: 'sequenceNumber',
                          },
                          {
                              header: 'Time',
                              accessorKey: 'time',
                          },
                          {
                              header: 'Transaction Count',
                              accessorKey: 'transactionCount',
                          },
                      ],
                  }
                : null,
        [checkpointsData]
    );

    return (
        <div>
            {checkpointsTable ? (
                <TableCard
                    data={checkpointsTable.data}
                    columns={checkpointsTable.columns}
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
                    label="Checkpoints"
                    data={checkpointsData}
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
