export interface AmoebaCategory {
  id?: number
  amoeba_id: number
  category_type: 'income' | 'expense'
  name: string
  desc: string
  sort_order: number
}

export interface AmoebaCategoryInput {
  category_type: 'income' | 'expense'
  name: string
  desc: string
  sort_order: number
}

export interface CategoryList {
  income: AmoebaCategory[]
  expense: AmoebaCategory[]
}

export interface SaveCategoriesInput {
  income: AmoebaCategoryInput[]
  expense: AmoebaCategoryInput[]
}
