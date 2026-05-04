export interface Amoeba {
  id?: number
  name: string
  amoeba_type: string
  leader: string
  parent_id?: number
  status: string
  created_at: string
  updated_at: string
}

export interface AmoebaInput {
  name: string
  amoeba_type: string
  leader: string
  parent_id?: number
}
