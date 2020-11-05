// Copyright 2017-2020 @polkadot/app-contracts authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

/* eslint-disable camelcase */

import { PrefabWasmModule } from '@polkadot/types/interfaces';

import React, { useMemo } from 'react';
import { Option } from '@polkadot/types';
import { InfoForInput } from '@polkadot/react-components';
import { useApi, useCall } from '@polkadot/react-hooks';
import { isHex } from '@polkadot/util';

import { useTranslation } from '../translate';

interface Props {
  codeHash?: string | null;
  onChange: React.Dispatch<boolean>;
}

function ValidateCode ({ codeHash, onChange }: Props): React.ReactElement<Props> | null {
  const { api } = useApi();
  const { t } = useTranslation();
  const codeStorage = useCall<Option<PrefabWasmModule>>((api.query.contracts || api.query.contract).codeStorage, [codeHash]);
  const [isValidHex, isValid] = useMemo(
    (): [boolean, boolean] => {
      const isValidHex = !!codeHash && isHex(codeHash) && codeHash.length === 66;
      const isStored = !!codeStorage && codeStorage.isSome;
      const isValid = isValidHex && isStored;

      onChange(isValid);

      return [
        isValidHex,
        isValid
      ];
    },
    [codeHash, codeStorage, onChange]
  );

  if (isValid || !isValidHex) {
    return null;
  }

  return (
    <InfoForInput type='error'>
      {
        isValidHex
          ? t('Unable to find on-chain WASM code for the supplied codeHash')
          : t('The codeHash is not a valid hex hash')
      }
    </InfoForInput>
  );
}

export default React.memo(ValidateCode);