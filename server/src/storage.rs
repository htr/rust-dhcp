//! The trait user must implement to provide a persistent lease storage for the DHCP server.

use std::net::Ipv4Addr;

use lease::Lease;

/// Errors generated by the `Storage` trait methods.
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Client getting error: {}", _0)]
    GetClient(String),
    #[fail(display = "Client adding error: {}", _0)]
    AddClient(String),
    #[fail(display = "Client deleting error: {}", _0)]
    DeleteClient(String),

    #[fail(display = "Lease getting error: {}", _0)]
    GetLease(String),
    #[fail(display = "Lease adding error: {}", _0)]
    AddLease(String),
    #[fail(display = "Lease updating error: {}", _0)]
    UpdateLease(String),

    #[fail(display = "Frozen address checking error: {}", _0)]
    CheckFrozen(String),
    #[fail(display = "Frozen address adding error: {}", _0)]
    AddFrozen(String),

    #[fail(display = "Another error: {}", _0)]
    Other(String),
}

/// Must be implemented by the DHCP server library user.
///
/// Be sure your storage is `ACID`.
pub trait Storage
where
    Self: Sync + Send,
{
    /// Must return the client ID if the client is associated with the given address.
    ///
    /// # Errors
    /// Must return `Error::GetClient(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn get_client(
        &self,
        address: &Ipv4Addr,
    ) -> Result<Option<Vec<u8>>, Error>;

    /// Must associate the client with the given address.
    ///
    /// # Errors
    /// Must return `Error::AddClient(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn add_client(
        &mut self,
        address: &Ipv4Addr,
        client_id: &[u8],
    ) -> Result<(), Error>;

    /// Must disassociate the client from the given address.
    ///
    /// # Errors
    /// Must return `Error::DeleteClient(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn delete_client(
        &mut self,
        address: &Ipv4Addr,
    ) -> Result<(), Error>;

    /// Must return the address lease of the given client if the lease exists.
    ///
    /// # Errors
    /// Must return `Error::GetLease(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn get_lease(
        &self,
        client_id: &[u8],
    ) -> Result<Option<Lease>, Error>;

    /// Must associate the client with the given lease.
    ///
    /// # Errors
    /// Must return `Error::AddLease(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn add_lease(
        &mut self,
        client_id: &[u8],
        lease: Lease,
    ) -> Result<(), Error>;

    /// Must update the lease associated with the given client if the lease exists.
    ///
    /// May return the updated lease.
    ///
    /// # Errors
    /// Must return `Error::UpdateLease(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn update_lease(
        &mut self,
        client_id: &[u8],
        action: &mut FnMut(&mut Lease) -> (),
    ) -> Result<Option<Lease>, Error>;

    /// Must return `true` if the given address has been frozen, `false` otherwise.
    ///
    /// # Errors
    /// Must return `Error::CheckFrozen(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn check_frozen(
        &self,
        address: &Ipv4Addr,
    ) -> Result<bool, Error>;

    /// Must mark the address as frozen due to a client DHCPDECLINE report.
    ///
    /// # Errors
    /// Must return `Error::AddFrozen(desc)` if there is a database I/O error
    /// or `Error::Other(desc)` if you want to report another error (e.g. connection).
    fn add_frozen(
        &mut self,
        address: &Ipv4Addr,
    ) -> Result<(), Error>;
}