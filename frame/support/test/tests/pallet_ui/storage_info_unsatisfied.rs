#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{Hooks, StorageValue, Weight};
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>, Weight> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[derive(codec::Encode, codec::Decode, scale_info::TypeInfo)]
	struct Bar;

	#[pallet::storage]
	type Foo<T> = StorageValue<_, Bar>;
}

fn main() {
}
