/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /** An owner of an account. */
  AccountOwner: { input: any; output: any; }
  /** The unique identifier (UID) of a chain. This is currently computed as the hash value of a ChainDescription. */
  ChainId: { input: any; output: any; }
};

/** Empty additional fields */
export type EmptyFields = {
  __typename?: 'EmptyFields';
};

/** An account. */
export type FungibleAccount = {
  /** Chain ID of the account */
  chainId: Scalars['ChainId']['input'];
  /** Owner of the account */
  owner: Scalars['AccountOwner']['input'];
};

export type Mutation = {
  __typename?: 'Mutation';
  /** Claims an airdrop. */
  airDropClaim: Array<Scalars['Int']['output']>;
};


export type MutationAirDropClaimArgs = {
  destination: FungibleAccount;
  signature: Scalars['String']['input'];
};

export type AirDropClaimMutationVariables = Exact<{
  destination: FungibleAccount;
  signature: Scalars['String']['input'];
}>;


export type AirDropClaimMutation = { __typename?: 'Mutation', airDropClaim: Array<number> };


export const AirDropClaimDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"AirDropClaim"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"destination"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"FungibleAccount"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"signature"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"airDropClaim"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"destination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"destination"}}},{"kind":"Argument","name":{"kind":"Name","value":"signature"},"value":{"kind":"Variable","name":{"kind":"Name","value":"signature"}}}]}]}}]} as unknown as DocumentNode<AirDropClaimMutation, AirDropClaimMutationVariables>;